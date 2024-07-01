use polars::{frame::DataFrame, prelude::LazyFrame};
use polars_sql::SQLContext;
use ratatui::widgets::*;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default] Normal,
    Filter,
    Query,
    Order,
    Table,
    Schema
}

#[derive(Default)]
pub struct App {
    pub df: DataFrame,
    pub sql_ctx: SQLContext,
    pub exit: bool,
    pub schema_scroller: Scroller,
    pub table_scroller: Scroller,
    pub filter_input: String,
    pub input: String,
    pub character_index: usize,
    pub input_mode: InputMode,
}

#[derive(Default, Debug)]
pub struct Scroller {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl App {
    pub fn new(df: DataFrame, sql_ctx: SQLContext) -> App {
        App {
            exit: false,
            df,
            sql_ctx,
            schema_scroller: Scroller::default(),
            table_scroller: Scroller::default(),
            input: String::new(),
            filter_input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        match self.input_mode {
            InputMode::Filter => {
                self.filter_input.insert(index, new_char);
            }
            _ => {
                self.input.insert(index, new_char);
            }
        }
        self.move_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        match self.input_mode {
            InputMode::Filter => {
                self.filter_input
                    .char_indices()
                    .map(|(i, _)| i)
                    .nth(self.character_index)
                    .unwrap_or(self.filter_input.len())
            }
            _ => {
                self.input
                    .char_indices()
                    .map(|(i, _)| i)
                    .nth(self.character_index)
                    .unwrap_or(self.input.len())
            }
        }
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            match self.input_mode {
                InputMode::Filter => {
                    let before_char_to_delete = self.filter_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.filter_input.chars().skip(current_index);

                    self.filter_input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                _ => {
                    let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.input.chars().skip(current_index);

                    self.input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
            }
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        match self.input_mode {
            InputMode::Filter => {
                new_cursor_pos.clamp(0, self.filter_input.chars().count())
            }
            _ => {
                new_cursor_pos.clamp(0, self.input.chars().count())
            }
        }
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        match self.input_mode {
            InputMode::Filter => {
                self.df = self.sql_ctx.execute(format!("select * from df where {}", self.filter_input).as_str()).and_then(LazyFrame::collect).unwrap();
                self.filter_input.clear();
            }
            _ => {
                self.input.clear();
            }
        }
        self.reset_cursor();
    }
}
