use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;

use crate::{app::{App, InputMode}, handler::{handle_filter_key_event, handle_schema_key_event}};

#[derive(Debug, Default)]
pub struct EventHandler {}

impl EventHandler {
    pub fn handle_events(&mut self, app: &mut App) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, app)
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match app.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('F') => {
                    app.input_mode = InputMode::Filter;
                }
                KeyCode::Char('Q') => {
                    app.input_mode = InputMode::Query;
                }
                KeyCode::Char('O') => {
                    app.input_mode = InputMode::Order;
                }
                KeyCode::Char('T') => {
                    app.input_mode = InputMode::Table;
                }
                KeyCode::Char('S') => {
                    app.input_mode = InputMode::Schema;
                }
                KeyCode::Char('q') => {
                    app.exit = true;
                }
                _ => {}
            }
            InputMode::Filter if key_event.kind == KeyEventKind::Press => match key_event.code {
                _ => handle_filter_key_event(key_event, app)
            }
            InputMode::Filter => {}
            InputMode::Query => todo!(),
            InputMode::Order => todo!(),
            InputMode::Table => todo!(),
            InputMode::Schema => handle_schema_key_event(key_event, app),
        }
    }
}
