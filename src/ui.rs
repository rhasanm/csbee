use functions::functions_schema;
use ratatui::{prelude::*, Frame};
use schema::schema_widget;
use table::table_schema;

mod schema;
mod functions;
mod table;

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let schema = app.df.schema();
    let area = frame.size();

    let body = Layout::vertical([
        Constraint::Percentage(15), // functions
        Constraint::Percentage(85) // table & Schema
    ]);

    let [fn_area, tbl_area] = body.areas(area);

    let tbl_chunks = Layout::horizontal([
        Constraint::Percentage(80), // table
        Constraint::Percentage(20), // schema
    ])
    .split(tbl_area);

    // let chunks = Layout::vertical([
    //     Constraint::Min(1),
    //     Constraint::Percentage(90),
    //     // Constraint::Percentage(25),
    //     // Constraint::Percentage(25),
    //     // Constraint::Percentage(25),
    // ])
    // .split(area);

    let (schema_widget, schema_scrollbar)  = schema_widget(schema.clone());
    let schema_widget = schema_widget.scroll((app.schema_scroller.vertical_scroll as u16, 0));
    app.schema_scroller.vertical_scroll_state = app.schema_scroller.vertical_scroll_state.content_length(schema.clone().len());
    // app.schema_scroller.horizontal_scroll_state = app.schema_scroller.horizontal_scroll_state.content_length(long_line.len());

    // ==================== FUNCTIONS
    frame.render_widget(functions_schema(), fn_area);
    // ==================== TABLE
    frame.render_widget(table_schema(), tbl_chunks[0]);
    // ==================== SCHEMA
    frame.render_widget(schema_widget, tbl_chunks[1]);
    frame.render_stateful_widget(
        schema_scrollbar,
        tbl_chunks[1],
        &mut app.schema_scroller.vertical_scroll_state,
    );
}
