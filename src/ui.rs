use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

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
    render_title(frame, chunks[0]);
    render_main_content(frame, chunks[1], app);
    render_info(frame, chunks[2], app);
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(Line::from("Cosmo Text Editor").centered(), area);
}

fn render_main_content(frame: &mut Frame, area: Rect, app: &mut App) {
    // Layout
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(area);
    render_lines(frame, chunks[0], app);
    render_editing_line(frame, chunks[1], app);
}

fn render_lines(frame: &mut Frame, area: Rect, app: &mut App) {
    let list = List::new(app.lines_vec().clone())
        .block(Block::new().borders(Borders::ALL))
        .highlight_symbol(">> ")
        .highlight_style(if matches!(app.mode(), Mode::Editing) {
            Style::new().fg(Color::DarkGray)
        } else {
            Style::default()
        });
    frame.render_stateful_widget(list, area, app.list_state_mut());
}

fn render_editing_line(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_editing = matches!(app.mode(), Mode::Editing);
    let style = if is_editing {
        Style::new().fg(Color::White)
    } else {
        Style::new().fg(Color::DarkGray)
    };
    let current_line = app.current_line();
    let column_pos = app.column_pos() as usize;
    let current_line = if !current_line.is_empty() {
        Line::from(vec![
            Span::from(current_line[0..column_pos].to_owned()),
            Span::styled(
                current_line[column_pos..(column_pos + 1)].to_owned(),
                if is_editing {
                    Style::new().fg(Color::Black).bg(Color::White)
                } else {
                    Style::new()
                },
            ),
            Span::from(current_line[(column_pos + 1)..].to_owned()),
        ])
    } else {
        Line::from(current_line)
    };
    let editing_line = Paragraph::new(current_line)
        .block(Block::new().borders(Borders::ALL))
        .style(style);
    frame.render_widget(editing_line, area);
}

fn render_info(frame: &mut Frame, area: Rect, app: &App) {
    let pos = format!(
        "Line (↑↓): {} | Column (←→): {} | Mode (Tab): {} | Exit (ESC)",
        app.line_pos() + 1,
        app.column_pos() + 1,
        app.mode(),
    );
    let line = Line::from(pos).centered();
    frame.render_widget(line, area);
}
