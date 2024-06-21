use polars::frame::DataFrame;
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

#[derive(Debug, Default)]
pub struct App {
    pub df: DataFrame,
    pub exit: bool,
    pub schema_scroller: SchemaScroller,
    pub filter_input: String,
    /// Current value of the input box
    pub input: String,
    /// Position of cursor in the editor area.
    pub character_index: usize,
    /// Current input mode
    pub input_mode: InputMode,
}

#[derive(Default, Debug)]
pub struct SchemaScroller {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl App {
    pub fn new(df: DataFrame) -> App {
        App {
            exit: false,
            df,
            schema_scroller: SchemaScroller::default(),
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
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            match self.input_mode {
                InputMode::Filter => {
                    // Getting all characters before the selected character.
                    let before_char_to_delete = self.filter_input.chars().take(from_left_to_current_index);
                    // Getting all characters after selected character.
                    let after_char_to_delete = self.filter_input.chars().skip(current_index);

                    // Put all characters together except the selected one.
                    // By leaving the selected one out, it is forgotten and therefore deleted.
                    self.filter_input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                _ => {
                    // Getting all characters before the selected character.
                    let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                    // Getting all characters after selected character.
                    let after_char_to_delete = self.input.chars().skip(current_index);

                    // Put all characters together except the selected one.
                    // By leaving the selected one out, it is forgotten and therefore deleted.
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
                self.filter_input.clear();
            }
            _ => {
                self.input.clear();
            }
        }
        self.reset_cursor();
    }
}
