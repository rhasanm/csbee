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
            KeyCode::Char('q') => {
                (*app).exit = true;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.schema_scroller.vertical_scroll =
                    app.schema_scroller.vertical_scroll.saturating_add(1);
                app.schema_scroller.vertical_scroll_state = app
                    .schema_scroller
                    .vertical_scroll_state
                    .position(app.schema_scroller.vertical_scroll);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.schema_scroller.vertical_scroll =
                    app.schema_scroller.vertical_scroll.saturating_sub(1);
                app.schema_scroller.vertical_scroll_state = app
                    .schema_scroller
                    .vertical_scroll_state
                    .position(app.schema_scroller.vertical_scroll);
            }
            KeyCode::Char('h') | KeyCode::Left => {
                app.schema_scroller.horizontal_scroll =
                    app.schema_scroller.horizontal_scroll.saturating_sub(1);
                app.schema_scroller.horizontal_scroll_state = app
                    .schema_scroller
                    .horizontal_scroll_state
                    .position(app.schema_scroller.horizontal_scroll);
            }
            KeyCode::Char('l') | KeyCode::Right => {
                app.schema_scroller.horizontal_scroll =
                    app.schema_scroller.horizontal_scroll.saturating_add(1);
                app.schema_scroller.horizontal_scroll_state = app
                    .schema_scroller
                    .horizontal_scroll_state
                    .position(app.schema_scroller.horizontal_scroll);
            }
            _ => {}
        }
    }
}
