use ratatui::widgets::*;

use crate::app::App;

pub fn table_schema(app: &mut App) -> Paragraph {
    Paragraph::new("Hello")
        .block(Block::bordered().title("Table"))
        .scroll((app.table_scroller.vertical_scroll as u16, 0))
}
