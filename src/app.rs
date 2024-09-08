use polars::{frame::DataFrame, prelude::LazyFrame};
use polars_sql::SQLContext;
use ratatui::widgets::*;

use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Filter,
    Query,
    Order,
    Table,
    Schema,
}

#[derive(Default)]
pub struct App {
    pub df: DataFrame,
    pub sql_ctx: SQLContext,
    pub exit: bool,
    pub schema_scroller: Scroller,
    pub table_scroller: Scroller,
    pub filter_input: String,
    pub query_input: String,
    pub order_input: String,
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
            order_input: String::new(),
            filter_input: String::new(),
            query_input: String::new(),
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
            InputMode::Query => {
                self.query_input.insert(index, new_char);
            }
            InputMode::Order => {
                self.order_input.insert(index, new_char);
            }
            _ => {}
        }
        self.move_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        match self.input_mode {
            InputMode::Filter => self
                .filter_input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.character_index)
                .unwrap_or(self.filter_input.len()),
            InputMode::Query => self
                .query_input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.character_index)
                .unwrap_or(self.query_input.len()),
            InputMode::Order => self
                .order_input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.character_index)
                .unwrap_or(self.order_input.len()),
            _ => 0,
        }
    }

    pub fn copy_schema(&mut self) {
        let mut ctx = ClipboardContext::new().unwrap();

        let mut formatted_schema = String::new();
        for field in self.df.schema().iter_fields() {
            formatted_schema.push_str(&format!("{}: {}\n", field.name(), field.data_type()));
        }

        ctx.set_contents(formatted_schema.to_owned()).unwrap();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            match self.input_mode {
                InputMode::Query => {
                    let before_char_to_delete =
                        self.query_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.query_input.chars().skip(current_index);

                    self.query_input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                InputMode::Filter => {
                    let before_char_to_delete =
                        self.filter_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.filter_input.chars().skip(current_index);

                    self.filter_input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                InputMode::Order => {
                    let before_char_to_delete =
                        self.order_input.chars().take(from_left_to_current_index);
                    let after_char_to_delete = self.order_input.chars().skip(current_index);

                    self.order_input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                _ => {}
            }
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        match self.input_mode {
            InputMode::Query => new_cursor_pos.clamp(0, self.query_input.chars().count()),
            InputMode::Filter => new_cursor_pos.clamp(0, self.filter_input.chars().count()),
            InputMode::Order => new_cursor_pos.clamp(0, self.order_input.chars().count()),
            _ => 0,
        }
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn submit_message(&mut self) {
        match self.input_mode {
            InputMode::Query => {
                self.df = self
                    .sql_ctx
                    .execute(&self.query_input)
                    .and_then(LazyFrame::collect)
                    .unwrap();
                self.query_input.clear();
            }
            InputMode::Filter => {
                self.df = self
                    .sql_ctx
                    .execute(format!("select * from df where {}", self.filter_input).as_str())
                    .and_then(LazyFrame::collect)
                    .unwrap();
                self.filter_input.clear();
            }
            InputMode::Order => {
                self.df = self
                    .sql_ctx
                    .execute(format!("select * from df order by {}", self.order_input).as_str())
                    .and_then(LazyFrame::collect)
                    .unwrap();
                self.order_input.clear();
            }
            _ => {}
        }
        self.reset_cursor();
    }
}
