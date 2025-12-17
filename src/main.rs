mod app;
mod keyboard;
mod ui;

use std::io;

use crate::{
    keyboard::{handle_keyboard, Action},
    ui::draw_ui,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw_ui)?;
        match handle_keyboard()? {
            Action::None => {}
            Action::Exit => break,
            Action::Move(direction) => todo!(),
            Action::AddChar(ch) => todo!(),
            Action::RemoveChar => todo!(),
        }
    }
    ratatui::restore();
    Ok(())
}
