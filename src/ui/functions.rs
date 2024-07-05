use crate::app::*;
use ratatui::{prelude::*, widgets::*};

pub fn functions_schema(app: &mut App) -> (Paragraph, Paragraph, Paragraph) {
    let filter_style = match app.input_mode {
        InputMode::Filter => Style::default().fg(Color::Yellow),
        _ => Style::default()
    };
    let query_style = match app.input_mode {
        InputMode::Query => Style::default().fg(Color::Yellow),
        _ => Style::default()
    };
    let order_style = match app.input_mode {
        InputMode::Order => Style::default().fg(Color::Yellow),
        _ => Style::default()
    };
    let query = Paragraph::new(app.query_input.as_str())
        .style(Style::default())
        .block(Block::bordered().title("Query").border_style(query_style));
    let filter = Paragraph::new(app.filter_input.as_str())
        .style(Style::default())
        .block(Block::bordered().title("Filter").border_style(filter_style));
    let order = Paragraph::new(app.order_input.as_str())
        .style(Style::default())
        .block(Block::bordered().title("Order By").border_style(order_style));

    (query, filter, order)
}
