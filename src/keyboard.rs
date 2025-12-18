use std::io;

use crossterm::event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crossterm::event::KeyEventKind;

use crossterm::event::Event;


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
    ChangeMode,
    AddChar(char),
    RemoveChar,
}

pub fn handle_keyboard() -> io::Result<Action> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(key),
        Event::Resize(_, _) => Ok(Action::None),
        _ => Ok(Action::Exit),
    }
}

pub fn handle_key(key: KeyEvent) -> io::Result<Action> {
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
        KeyCode::Tab => Ok(Action::ChangeMode),
        // Pressing the escape key will allow the user to exit.
        KeyCode::Esc => Ok(Action::Exit),
        // TODO: Implement these later.
        // KeyCode::BackTab => todo!(),
        // KeyCode::Delete => todo!(),
        // KeyCode::Home => todo!(),
        // KeyCode::End => todo!(),
        // KeyCode::PageUp => todo!(),
        // KeyCode::PageDown => todo!(),
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
