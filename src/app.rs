use polars::frame::DataFrame;

#[derive(Debug, Default)]
pub struct App {
    pub df: DataFrame,
    pub exit: bool,
}

impl App {
    pub fn new(df: DataFrame) -> App {
        App {exit: false, df}
    }
}