use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    match app.mode() {
        Mode::Reading | Mode::Editing => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    // Stretches to fill available space.
                    Constraint::Min(0),
                    Constraint::Percentage(10),
                ])
                .split(frame.area());
            render_main_content(frame, chunks[0], app);
            render_info(frame, chunks[1], app);
        }
        Mode::Exiting => {
            render_exiting_popup(frame);
        }
    }
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
    let numbered_lines = app
        .lines_vec()
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{: >3}. {}", i + 1, line));
    let list = List::new(numbered_lines)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title(" Cosmo Text Editor ")
                .title_alignment(Alignment::Center),
        )
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
        "Line (↑↓): {} | Column (←→): {} | Newline (Enter) | Mode (Tab): {} | Exit (ESC)",
        app.line_pos() + 1,
        app.column_pos() + 1,
        app.mode(),
    );
    let line = Line::from(pos).centered();
    frame.render_widget(line, area);
}

fn render_exiting_popup(frame: &mut Frame) {
    let container = center(
        frame.area(),
        Constraint::Percentage(50),
        Constraint::Percentage(50), // top and bottom border + content
    );
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(container);
    let prompt =
        Paragraph::new("Do you want to save your changes?").block(Block::bordered().title("Popup"));
    let options = Paragraph::new("Yes (Y/y) or No (N/n)");
    frame.render_widget(Clear, container);
    frame.render_widget(prompt, chunks[0]);
    frame.render_widget(options, chunks[1]);
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
