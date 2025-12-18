use std::io;

use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crossterm::event::KeyEventKind;

use crossterm::event::Event;

use crate::app::Mode;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Action {
    None,
    Exit,
    Move(Direction),
    ChangeMode(Mode),
    AddChar(char),
    RemoveChar,
    Save,
}

pub fn handle_keyboard(mode: &Mode) -> io::Result<Action> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match mode {
            Mode::Reading | Mode::Editing => handle_key(key, mode),
            Mode::Exiting => handle_exiting_key(key),
        },
        Event::Resize(_, _) => Ok(Action::None),
        _ => Ok(Action::Exit),
    }
}

pub fn handle_key(key: KeyEvent, mode: &Mode) -> io::Result<Action> {
    match key.code {
        // Pressing backspace deletes one character.
        KeyCode::Backspace => Ok(Action::RemoveChar),
        // Pressing enter creates a new line.
        KeyCode::Enter => Ok(Action::AddChar('\n')),
        // Arrow keys.
        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
            Ok(handle_arrow_key(key.code))
        }
        // When a character is entered, it added to the currently selected line.
        KeyCode::Char(ch) => Ok(Action::AddChar(ch)),
        // Pressing tab changes between editing and reading mode.
        KeyCode::Tab => Ok(Action::ChangeMode(match mode {
            Mode::Reading => Mode::Editing,
            Mode::Editing => Mode::Reading,
            Mode::Exiting => unreachable!(),
        })),
        // Pressing the escape key will allow the user to exit.
        KeyCode::Esc => Ok(Action::ChangeMode(Mode::Exiting)),
        _ => Ok(Action::None),
    }
}

fn handle_arrow_key(code: KeyCode) -> Action {
    match code {
        KeyCode::Up => Action::Move(Direction::Up),
        KeyCode::Down => Action::Move(Direction::Down),
        KeyCode::Left => Action::Move(Direction::Left),
        KeyCode::Right => Action::Move(Direction::Right),
        _ => unreachable!(),
    }
}

pub fn handle_exiting_key(key: KeyEvent) -> io::Result<Action> {
    if let KeyCode::Char(ch) = key.code {
        if ch.eq_ignore_ascii_case(&'Y') {
            return Ok(Action::Save);
        } else if ch.eq_ignore_ascii_case(&'N') {
            return Ok(Action::Exit);
        }
    }
    Ok(Action::None)
}
