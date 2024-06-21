use ratatui::{style::Stylize, widgets::Paragraph, Frame};

use crate::app::App;

pub fn ui(frame: &mut Frame, _app: &App) {
    let widget = Paragraph::new("Hello World").blue().on_white();
    frame.render_widget(widget, frame.size())
}