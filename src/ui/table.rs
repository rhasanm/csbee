use ratatui::{prelude::*, widgets::*};

pub fn table_schema() -> Block<'static> {
    Block::new()
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().fg(Color::White))
        .title("Block")
}
