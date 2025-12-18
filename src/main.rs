mod app;
mod keyboard;
mod ui;

use std::io;

use crate::{
    app::App,
    keyboard::{handle_keyboard, Action},
    ui::draw_ui,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new("temp/poem.txt")?;
    loop {
        terminal.draw(|frame| draw_ui(frame, &mut app))?;
        match handle_keyboard()? {
            Action::None => {}
            Action::Exit => break,
            Action::Move(direction) => match direction {
                keyboard::Direction::Up => app.move_previous_line(),
                keyboard::Direction::Down => app.move_next_line(),
                keyboard::Direction::Left => todo!(),
                keyboard::Direction::Right => todo!(),
            },
            Action::AddChar(ch) => todo!(),
            Action::RemoveChar => todo!(),
        }
    }
    ratatui::restore();
    Ok(())
}
