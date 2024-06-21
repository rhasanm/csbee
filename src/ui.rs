use ratatui::{prelude::*, Frame};
use schema::schema_widget;

mod schema;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let schema = app.df.schema();
    let area = frame.size();

    let (schema_widget, schema_scrollbar, mut schema_scrollbar_state)  = schema_widget(schema);

    frame.render_widget(schema_widget, area);
    frame.render_stateful_widget(
        schema_scrollbar,
        area.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut schema_scrollbar_state,
    );
}
