use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;

use crate::app::App;

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
        match key_event.code {
            KeyCode::Char('q') => { (*app).exit = true; },
            _ => {}
        }
    }
}