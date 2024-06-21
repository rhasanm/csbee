use crate::app::*;
use ratatui::{prelude::*, widgets::*};

pub fn functions_schema(app: &mut App) -> (Paragraph, Paragraph, Paragraph) {
    let filter_style = match app.input_mode {
        InputMode::Filter => Style::default().fg(Color::Yellow),
        _ => Style::default()
    };
    let query = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::bordered().title("Query"));
    let filter = Paragraph::new(app.filter_input.as_str())
        .style(filter_style)
        .block(Block::bordered().title("Filter"));
    let order = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::bordered().title("Order By"));

    (query, filter, order)
}
