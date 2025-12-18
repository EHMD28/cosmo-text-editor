mod app;
mod keyboard;
mod ui;

use std::io;

use crate::{
    app::{App, Mode},
    keyboard::{handle_keyboard, Action},
    ui::draw_ui,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new("temp/poem.txt")?;
    loop {
        terminal.draw(|frame| draw_ui(frame, &mut app))?;
        let is_editing = matches!(app.mode(), Mode::Editing);
        match handle_keyboard()? {
            Action::None => {}
            Action::Exit => break,
            Action::Move(direction) => match direction {
                keyboard::Direction::Up if !is_editing => app.move_previous_line(),
                keyboard::Direction::Down if !is_editing => app.move_next_line(),
                keyboard::Direction::Left if is_editing => {
                    app.move_previous_column();
                }
                keyboard::Direction::Right if is_editing => {
                    app.move_next_column();
                }
                // Do nothing.
                _ => {}
            },
            Action::ChangeMode => app.switch_mode(),
            Action::AddChar(ch) if is_editing => app.insert_char(ch),
            Action::RemoveChar if is_editing => app.remove_char(),
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}
