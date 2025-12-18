mod app;
mod keyboard;
mod ui;

use std::io::{self, Stdout};

use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{
    app::{App, Mode},
    keyboard::{handle_keyboard, Action},
    ui::draw_ui,
};

fn main() -> io::Result<()> {
    let path = "temp/poem.txt";
    let mut terminal = ratatui::init();
    let mut app = App::new(path)?;
    // Returns true if the user wants to save, otherwise returns false.
    let do_save = run_app(&mut terminal, &mut app)?;
    ratatui::restore();
    if do_save {
        app.save_to_file(path)?;
        println!("Saved");
    }
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<bool, io::Error> {
    loop {
        terminal.draw(|frame| draw_ui(frame, app))?;
        let is_editing = matches!(app.mode(), Mode::Editing);
        match handle_keyboard(app.mode())? {
            Action::None => {}
            Action::Exit => return Ok(false),
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
            Action::ChangeMode(mode) => app.set_mode(mode),
            Action::AddChar(ch) if is_editing => app.insert_char(ch),
            Action::RemoveChar if is_editing => app.remove_char(),
            Action::Save => {
                return Ok(true);
            }
            _ => {}
        }
    }
}
