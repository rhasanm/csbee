use functions::functions_schema;
use ratatui::{
    prelude::*,
    widgets::{Scrollbar, ScrollbarOrientation},
    Frame,
};
use schema::schema_widget;
use table::table_schema;

mod functions;
mod schema;
mod table;

use crate::app::{App, InputMode};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let schema = app.df.schema();
    let area = frame.size();

    let body = Layout::vertical([
        Constraint::Percentage(8),  // functions
        Constraint::Percentage(92), // table & Schema
    ]);

    let [fn_area, tbl_area] = body.areas(area);

    let fn_chunks = Layout::horizontal([
        Constraint::Percentage(45), // sql query
        Constraint::Percentage(30), // filter
        Constraint::Percentage(25), // order by
        Constraint::Min(0),
    ])
    .split(fn_area);

    let tbl_chunks = Layout::horizontal([
        Constraint::Percentage(80), // table
        Constraint::Percentage(20), // schema
    ])
    .split(tbl_area);

    let (schema_widget, schema_scrollbar) = schema_widget(schema.clone(), app);
    let schema_widget = schema_widget.scroll((app.schema_scroller.vertical_scroll as u16, 0));
    // .scroll((0, app.schema_scroller.horizontal_scroll as u16));

    app.schema_scroller.vertical_scroll_state = app
        .schema_scroller
        .vertical_scroll_state
        .content_length(schema.clone().len());
    app.table_scroller.vertical_scroll_state = app
        .table_scroller
        .vertical_scroll_state
        .content_length(app.df.height());
    // app.schema_scroller.horizontal_scroll_state = app.schema_scroller.horizontal_scroll_state.content_length(schema.clone().len());

    // ==================== FUNCTIONS
    match app.input_mode {
        InputMode::Filter => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                fn_chunks[1].x + app.character_index as u16 + 1,
                fn_chunks[1].y + 1,
            );
        }
        _ => {}
    }
    let (query_widget, filter_widget, order_widget) = functions_schema(app);

    frame.render_widget(query_widget, fn_chunks[0]);
    frame.render_widget(filter_widget, fn_chunks[1]);
    frame.render_widget(order_widget, fn_chunks[2]);
    // ==================== TABLE
    frame.render_widget(table_schema(app), tbl_chunks[0]);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        tbl_chunks[0],
        &mut app.table_scroller.vertical_scroll_state,
    );
    // ==================== SCHEMA
    frame.render_widget(schema_widget, tbl_chunks[1]);
    frame.render_stateful_widget(
        schema_scrollbar,
        tbl_chunks[1],
        &mut app.schema_scroller.vertical_scroll_state,
    );
    // frame.render_stateful_widget(
    //     Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
    //         .thumb_symbol("🬋")
    //         .end_symbol(None),
    //     tbl_chunks[1],
    //     &mut app.schema_scroller.horizontal_scroll_state,
    // );
}
