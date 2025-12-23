use core::num;
use std::{
    cmp::min,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use ratatui::{layout::Rect, widgets::ListState};
use unicode_segmentation::UnicodeSegmentation;

struct CursorPosition {
    line: u16,
    column: u16,
}

pub enum Mode {
    Reading,
    Editing,
    Exiting,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Reading => "Reading",
                Mode::Editing => "Editing",
                Mode::Exiting => "Exiting",
            }
        )
    }
}

/// Used for representing the current state of the app.
pub struct App {
    /// The lines of the file being currently edited.
    lines: Vec<String>,
    /// The current state of the `List` representing the lines of the file
    list_state: ListState,
    /// The current line being edited.
    current_line: String,
    /// The offset of the current line (used for horizontal scrolling).
    offset: usize,
    /// Current row and column being viewed/edited.
    position: CursorPosition,
    /// The current mode of the app (reading, editing, or exiting).
    mode: Mode,
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
        let first_line = lines.first().unwrap_or(&String::new()).to_owned();
        Ok(App {
            lines,
            current_line: first_line,
            position: CursorPosition { line: 0, column: 0 },
            list_state: ListState::default().with_selected(Some(0)),
            mode: Mode::Reading,
            offset: 0,
        })
    }

    /// Selects line number `line`, starting from 0.
    fn select_line(&mut self, line_num: u16) {
        self.position.line = line_num;
        self.position.column = 0;
        self.offset = 0;
        let line_num = line_num as usize;
        self.list_state.select(Some(line_num));
        self.current_line = self.lines[line_num].to_owned();
    }

    fn select_column(&mut self, column_num: u16) {
        self.position.column = column_num;
        let past_end_of_line = self.current_line.len() == column_num.into();
        let previous_column = u16::saturating_sub(self.column_pos(), 1);
        let is_space = self.char_at(previous_column.into()) == ' ';
        if past_end_of_line {
            if !is_space {
                self.current_line.push(' ');
            } else {
                self.move_previous_column();
            }
        }
    }

    fn char_at(&mut self, n: usize) -> char {
        self.current_line.chars().nth(n).unwrap_or_default()
    }

    /// Selects the next line after the currently selected line. If there is no line after the
    /// current line, then the current line will remain selected.
    pub fn move_next_line(&mut self) {
        let target = min(self.lines.len() - 1, (self.line_pos() + 1).into());
        // if usize::from(target) == self.lines.len() {
        //     self.lines.push(String::new());
        // }
        self.select_line(target as u16);
    }

    /// Selects the line before the currently selected line. If no line is before the current line,
    /// then the current line will remain selected.
    pub fn move_previous_line(&mut self) {
        let target = u16::saturating_sub(self.line_pos(), 1);
        self.select_line(target);
    }

    pub fn move_next_column(&mut self) {
        let target = self.column_pos() + 1;
        self.select_column(target);
    }

    pub fn move_previous_column(&mut self) {
        let target = u16::saturating_sub(self.column_pos(), 1);
        self.select_column(target);
    }

    pub fn insert_char(&mut self, ch: char) {
        // If the cursor is at the end of a line, then insert a new column.
        let target: usize = if self.current_line.len() + 1 == self.column_pos().into() {
            (self.column_pos() + 1).into()
        } else {
            self.column_pos().into()
        };
        self.current_line.insert(target, ch);
        self.move_next_column();
    }

    pub fn remove_char(&mut self) {
        self.current_line.remove(self.column_pos().into());
        self.move_previous_column();
    }

    pub fn insert_newline(&mut self) {
        self.lines
            .insert((self.line_pos() + 1).into(), String::from(" "));
    }

    pub fn set_mode(&mut self, mode: Mode) {
        // When transitioning from editing to reading, update the line that was being edited.
        if matches!(self.mode, Mode::Editing) && matches!(mode, Mode::Reading) {
            let line_pos = self.line_pos() as usize;
            self.lines[line_pos] = self.current_line.to_owned();
        }
        self.mode = mode;
    }

    /// Returns a tuple representing the start (inclusive) and end (inclusive) for the current line.
    /// This allows for horizontal scrolling.
    pub fn calculate_offset(&mut self, area: Rect) -> (usize, usize) {
        // TODO: only scroll when reaching the start/end of a line.
        // The border around the editing line has two vertical bars on each side.
        let border_width = 2;
        // The number of columns in the editing line.
        let num_columns = usize::from(area.width - border_width);
        let column_pos = usize::from(self.column_pos());
        // The leftmost visible column.
        let leftmost_column = self.offset;
        // The rightmost visible column.
        let rightmost_column = min(self.current_line_len() - 1, (num_columns - 1) + self.offset);
        if column_pos < leftmost_column {
            self.offset = self.offset.saturating_sub(1);
            (
                leftmost_column.saturating_sub(1),
                rightmost_column.saturating_sub(1),
            )
        } else if column_pos > rightmost_column {
            self.offset += 1;
            (leftmost_column + 1, rightmost_column + 1)
        } else {
            (leftmost_column, rightmost_column)
        }
    }

    /// Writes all the lines in self.lines to a file at the given path.
    pub fn save_to_file(&mut self, path: &str) -> io::Result<()> {
        let path = Path::new(path);
        let mut file = File::create(path)?;
        for line in self.lines.iter_mut() {
            line.push('\n');
            file.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn lines_vec(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn current_line(&self) -> &str {
        &self.current_line
    }

    pub fn current_line_len(&self) -> usize {
        self.current_line.graphemes(true).count()
    }

    /// Returns the currently selected line starting from 0.
    pub fn line_pos(&self) -> u16 {
        self.position.line
    }

    /// Returns the currently selected column starting from 0.
    pub fn column_pos(&self) -> u16 {
        self.position.column
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
