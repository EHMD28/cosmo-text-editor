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
    match App::new(path) {
        Ok(mut app) => {
            let do_save = run_app(&mut terminal, &mut app)?;
            ratatui::restore();
            if do_save {
                app.save_to_file(path)?;
                println!("Saved");
            }
        }
        Err(err) => {
            ratatui::restore();
            eprintln!("{err}");
        }
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
            Action::Save => return Ok(true),
            Action::Move(direction) => handle_movement(app, is_editing, direction),
            Action::ChangeMode(mode) => app.set_mode(mode),
            Action::AddChar(ch) if is_editing => app.insert_char(ch),
            Action::RemoveChar if is_editing => app.remove_char(),
            Action::AddLine => app.insert_newline(),
            _ => {}
        }
    }
}

fn handle_movement(app: &mut App, is_editing: bool, direction: keyboard::Direction) {
    match direction {
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
    }
}
