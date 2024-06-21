use ratatui::{prelude::*, Frame};
use schema::schema_widget;

mod schema;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let schema = app.df.schema();
    let area = frame.size();
    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Percentage(90),
        // Constraint::Percentage(25),
        // Constraint::Percentage(25),
        // Constraint::Percentage(25),
    ])
    .split(area);

    let (schema_widget, schema_scrollbar)  = schema_widget(schema.clone());
    let schema_widget = schema_widget.scroll((app.schema_scroller.vertical_scroll as u16, 0));
    app.schema_scroller.vertical_scroll_state = app.schema_scroller.vertical_scroll_state.content_length(schema.clone().len());
    // app.schema_scroller.horizontal_scroll_state = app.schema_scroller.horizontal_scroll_state.content_length(long_line.len());

    frame.render_widget(schema_widget, chunks[1]);
    frame.render_stateful_widget(
        schema_scrollbar,
        chunks[1],
        &mut app.schema_scroller.vertical_scroll_state,
    );
}
