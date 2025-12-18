use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, List},
    Frame,
};

use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            // Stretches to fill available space.
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(frame.area());
    frame.render_widget(title(), chunks[0]);
    frame.render_stateful_widget(file_lines(app), chunks[1], app.list_state_mut());
    frame.render_widget(info(), chunks[2]);
}

fn title<'a>() -> Line<'a> {
    Line::from("Cosmo Text Editor").centered()
}

fn file_lines<'a>(app: &App) -> List<'a> {
    List::new(app.lines().clone())
        .block(Block::new().borders(Borders::ALL))
        .highlight_symbol(">> ")
}

fn info<'a>() -> Line<'a> {
    Line::from("Line: 0 | Column: 0").centered()
}
