use crossterm::event::*;

use crate::app::{App, InputMode};

pub fn handle_schema_key_event(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
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
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

pub fn handle_filter_key_event(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Enter => app.submit_message(),
        KeyCode::Char(to_insert) => {
            app.enter_char(to_insert);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Esc => {
            app.filter_input = String::from("");
            app.reset_cursor();

            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

pub fn handle_table_key_event(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.table_scroller.vertical_scroll =
                app.table_scroller.vertical_scroll.saturating_add(1);
            app.table_scroller.vertical_scroll_state = app
                .table_scroller
                .vertical_scroll_state
                .position(app.table_scroller.vertical_scroll);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.table_scroller.vertical_scroll =
                app.table_scroller.vertical_scroll.saturating_sub(1);
            app.table_scroller.vertical_scroll_state = app
                .table_scroller
                .vertical_scroll_state
                .position(app.table_scroller.vertical_scroll);
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app.table_scroller.horizontal_scroll =
                app.table_scroller.horizontal_scroll.saturating_sub(1);
            app.table_scroller.horizontal_scroll_state = app
                .table_scroller
                .horizontal_scroll_state
                .position(app.table_scroller.horizontal_scroll);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            app.table_scroller.horizontal_scroll =
                app.table_scroller.horizontal_scroll.saturating_add(1);
            app.table_scroller.horizontal_scroll_state = app
                .table_scroller
                .horizontal_scroll_state
                .position(app.table_scroller.horizontal_scroll);
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

pub fn handle_query_key_event(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Enter => app.submit_message(),
        KeyCode::Char(to_insert) => {
            app.enter_char(to_insert);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Esc => {
            app.query_input = String::from("");
            app.reset_cursor();

            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

pub fn handle_order_key_event(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        KeyCode::Enter => app.submit_message(),
        KeyCode::Char(to_insert) => {
            app.enter_char(to_insert);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Esc => {
            app.order_input = String::from("");
            app.reset_cursor();
        
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}