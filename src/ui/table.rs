use ratatui::{style::{Color, Modifier, Style}, text::{Line, Span}, widgets::*};

use crate::app::App;

pub fn table_schema(app: &mut App) -> Paragraph {
    let height = app.df.height();
    let columns = app.df.get_columns();

    let mut text: Vec<Line> = Vec::with_capacity(height);

    for i in 0..height {
        let mut values: Vec<String> = Vec::with_capacity(columns.len());

        for col in columns {
            let value = col.get(i).unwrap().to_string();
            values.push(value);
        }

        let row_text = values.join(", ");

        let span = Span::styled(row_text, Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC));

        text.push(Line::from(span));
    }

    Paragraph::new(text)
        .block(Block::bordered().title("Table"))
        .scroll((app.table_scroller.vertical_scroll as u16, 0))
}