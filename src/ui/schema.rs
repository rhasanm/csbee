use polars::prelude::Schema;
use ratatui::{prelude::*, widgets::*};

pub fn schema_widget(
    schema: Schema,
) -> (
    ratatui::widgets::Paragraph<'static>,
    ratatui::widgets::Scrollbar<'static>,
) {
    let items: Vec<_> = schema
        .iter()
        .map(|(key, val)| {
            let formatted_str = format!("{}: {:?}", key, val);
            // ListItem::new(formatted_str)
            Line::from(formatted_str)
        })
        .collect();

    let widget = Paragraph::new(items.clone())
        .block(Block::bordered().title("Schema"))
        .style(Style::default().fg(Color::White));

    // let widget = List::new(items.clone())
    //     .block(Block::bordered().title("Schema"))
    //     .style(Style::default().fg(Color::White))
    //     .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    //     .highlight_symbol(">>")
    //     .repeat_highlight_symbol(true)
    //     .direction(ListDirection::TopToBottom);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    // let scrollbar_state = ScrollbarState::new(items.len()).position(0);

    (widget, scrollbar)
}
