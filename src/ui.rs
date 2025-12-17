use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, List},
    Frame,
};

pub fn draw_ui(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Percentage(96),
            Constraint::Length(1),
        ])
        .split(frame.area());
    frame.render_widget(title(), chunks[0]);
    frame.render_widget(text_area(), chunks[1]);
    frame.render_widget(info(), chunks[2]);
}

fn title<'a>() -> Line<'a> {
    Line::from("Cosmo Text Editor").centered()
}

fn text_area<'a>() -> List<'a> {
    let items = [
        "I met a traveller from an antique land",
        "Who said—\"Two vast and trunkless legs of stone",
        "Stand in the desert. . . . Near them, on the sand",
    ];
    let items = items
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{i}. {line}"));
    List::new(items).block(Block::new().borders(Borders::ALL))
}

fn info<'a>() -> Line<'a> {
    Line::from("Line: 0 | Column: 0").centered()
}
