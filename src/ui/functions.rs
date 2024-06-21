use crate::app::*;
use ratatui::{prelude::*, widgets::*};

pub fn functions_schema(app: &mut App) -> Paragraph {
    // Block::new()
    //     .border_type(BorderType::Rounded)
    //     .borders(Borders::ALL)
    //     .border_style(Style::default().fg(Color::White))
    //     .style(Style::default().fg(Color::White))
    //     .title("Block");

    let style = match app.input_mode {
        InputMode::Filter => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    };
    let input = Paragraph::new(app.input.as_str())
        .style(style)
        .block(Block::bordered().title("Input"));

    input
}
