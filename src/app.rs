use polars::frame::DataFrame;
use ratatui::widgets::*;

#[derive(Debug, Default)]
pub struct App {
    pub df: DataFrame,
    pub exit: bool,
    pub schema_scroller: SchemaScroller,
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
        }
    }
}
