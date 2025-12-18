use std::{
    cmp::{max, min},
    fs::File,
    io::{self, BufRead, BufReader, Lines, Seek, SeekFrom},
    path::Path,
};

use ratatui::widgets::ListState;

pub struct App {
    lines: Vec<String>,
    list_state: ListState,
}

impl App {
    /// Creates a new instance of `App` using the provided file path. If an error occurs when
    /// opening the file, it will will be returned.
    pub fn new(path: &str) -> io::Result<App> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        // This syntax is ew.
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(App {
            lines,
            list_state: ListState::default(),
        })
    }

    /// Returns the currently selected line starting from 0.
    pub fn current_line(&self) -> usize {
        self.list_state.selected().unwrap_or_default()
    }

    /// Selects line number `line`, starting from 0.
    fn select_line(&mut self, line: usize) {
        self.list_state.select(Some(line));
    }

    /// Selects the next line after the currently selected line. If there is no after the current
    /// line, then the current line will remain selected.
    pub fn move_next_line(&mut self) {
        if self.lines.len() == 0 {
            self.select_line(1);
        } else {
            let target = min(self.lines.len() - 1, self.current_line() + 1);
            self.select_line(target);
        }
    }

    /// Selects the line before the currently selected line. If no line is before the current line,
    /// then the current line will remain selected.
    pub fn move_previous_line(&mut self) {
        if self.current_line() == 0 {
            self.select_line(0);
        } else {
            let target = self.current_line() - 1;
            self.select_line(target);
        }
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}
