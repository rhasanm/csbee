#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {exit: false}
    }
}