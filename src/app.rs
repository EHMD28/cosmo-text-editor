use std::{
    cmp::min,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Write},
    path::{Path, PathBuf},
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
    /// The path of the file being currently edited
    path: PathBuf,
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
    pub fn new(path: &PathBuf) -> App {
        App {
            path: path.to_owned(),
            lines: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
            current_line: String::new(),
            offset: 0,
            position: CursorPosition { line: 0, column: 0 },
            mode: Mode::Reading,
        }
    }

    pub fn load_file(&mut self) -> io::Result<()> {
        let result = File::open(&self.path);
        match result {
            Ok(file) => {
                let reader = BufReader::new(file);
                // This syntax is eww.
                let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
                let first_line = lines.first().unwrap_or(&String::new()).to_owned();
                self.lines = lines;
                self.current_line = first_line;
                Ok(())
            }
            Err(err) => {
                if matches!(err.kind(), ErrorKind::NotFound) {
                    // In this case, just add a blank first line, otherwise just using the defaults
                    // from new().
                    let default_line = String::from("Welcome to Cosmo!");
                    self.lines.push(default_line.to_owned());
                    self.current_line = default_line;
                    Ok(())
                } else {
                    Err(err)
                }
            }
        }
    }

    /// Writes all the lines in self.lines to a file at the given path.
    pub fn save_to_file(&mut self, path: &Path) -> io::Result<()> {
        let mut file = File::create(path)?;
        for line in self.lines.iter_mut() {
            line.push('\n');
            file.write_all(line.as_bytes())?;
        }
        Ok(())
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
        let is_past_end_of_line = self.current_line_len() == column_num.into();
        let previous_column = self.column_pos().saturating_sub(1);
        let is_space = self.grapheme_at(previous_column.into()) == " ";
        if is_past_end_of_line {
            if !is_space {
                self.current_line.push(' ');
            } else {
                self.move_previous_column();
            }
        }
    }

    fn grapheme_at(&mut self, n: usize) -> &str {
        self.current_line.graphemes(true).nth(n).unwrap_or_default()
    }

    /// Selects the next line after the currently selected line. If there is no line after the
    /// current line, then the current line will remain selected.
    pub fn move_next_line(&mut self) {
        let target = min(self.lines.len() - 1, (self.line_pos() + 1).into());
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
        let target: usize = if self.current_line_len() + 1 == self.column_pos().into() {
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
        if self.lines.is_empty() {
            self.lines.push(String::from(" "));
        } else {
            self.lines
                .insert((self.line_pos() + 1).into(), String::from(" "));
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        // When transitioning from editing to reading, update the line that was being edited.
        if matches!(self.mode, Mode::Editing) && matches!(mode, Mode::Reading) {
            let line_pos = self.line_pos() as usize;
            if !self.lines.is_empty() {
                self.lines[line_pos] = self.current_line.to_owned();
            }
        }
        self.mode = mode;
    }

    /// Returns a tuple representing the start (inclusive) and end (inclusive) for the current line.
    /// This allows for horizontal scrolling.
    pub fn calculate_offset(&mut self, area: Rect) -> (usize, usize) {
        if self.current_line_len() == 0 {
            return (0, 0);
        }
        // The border around the editing line has two vertical bars on each side.
        let border_width = 2;
        // The number of columns in the editing line.
        let num_columns = usize::from(area.width - border_width);
        let column_pos = usize::from(self.column_pos());
        // The leftmost visible column.
        let leftmost_column = self.offset;
        // The rightmost visible column.
        let rightmost_column = min(
            self.current_line_len().saturating_sub(1),
            (num_columns.saturating_sub(1)) + self.offset,
        );
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
