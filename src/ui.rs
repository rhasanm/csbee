use ratatui::{
    style::{Color, Modifier, Style}, widgets::{Block, List, ListDirection, ListItem}, Frame
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let schema = app.df.schema();

    let items: Vec<_> = schema
        .iter()
        .map(|(key, val)| {
            let formatted_str = format!("{}: {:?}", key, val);
            ListItem::new(formatted_str)
        })
        .collect();

    let widget = List::new(items)
        .block(Block::bordered().title("Schema"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    frame.render_widget(widget, frame.size())
}
