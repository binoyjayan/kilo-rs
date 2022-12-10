use crossterm::cursor;
use crossterm::style;
use crossterm::terminal;
use crossterm::QueueableCommand;

use std::io;
use std::io::Write;
use std::time;
use std::time::Duration;

use crate::cursor::*;
use crate::data::*;
use crate::events::*;
use crate::window::*;

pub struct Screen {
    stdout: io::Stdout,
    window: Window,
    cursor: Position,
    editrows: Vec<EditRow>,
    rowoff: usize,
    coloff: usize,
    file: Option<String>,
    status_msg: String,
    status_time: time::Instant,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Screen {
    pub fn new(lines: &[String], file: Option<String>) -> crossterm::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Self {
            stdout: io::stdout(),
            // One row on the bottom for status bar
            window: Window::new(width, height - 2),
            cursor: Position::new(0, 0),
            editrows: Self::make_editrows(lines),
            rowoff: 0,
            coloff: 0,
            file,
            status_msg: String::from("Ctrl-Q = quit"),
            status_time: time::Instant::now(),
        })
    }

    pub fn make_editrows(lines: &[String]) -> Vec<EditRow> {
        let mut editrows = lines
            .iter()
            .map(|line| EditRow::new(line.to_string()))
            .collect::<Vec<EditRow>>();

        // Remove last line if it is empty
        if !editrows.is_empty() {
            let last = editrows.iter().last().unwrap();
            if last.chars.is_empty() {
                editrows.pop();
            }
        }
        editrows
    }

    pub fn open(&mut self) -> crossterm::Result<()> {
        terminal::enable_raw_mode()
    }

    pub fn clear(&mut self) -> crossterm::Result<()> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.scroll();
        self.clear()?;
        self.draw_rows()?;
        self.draw_status()?;
        self.draw_message()?;
        Ok(())
    }

    /*
     * To display each row at the column offset, coloff as an index into the
     * chars of each editrow displayed, and subtract the number of characters
     * that are to the left of the offset from the length of the row. When
     * subtracting coloff from the length, len can be a negative number,
     * meaning the user scrolled horizontally past the end of the line.
     * In that case, len would be 0 and nothing is displayed on that line.
     */
    pub fn draw_rows(&mut self) -> crossterm::Result<()> {
        for y in 0..self.window.height {
            let filerow = y as usize + self.rowoff;
            if filerow >= self.editrows.len() {
                if self.editrows.is_empty() && y == self.window.height / 3 {
                    self.show_welcome(y)?;
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(0, y))?
                        .queue(style::Print("~".to_string()))?;
                }
            } else {
                let colstart = self.coloff;
                // Handling horizontal scrolling
                let len = self.editrows[filerow]
                    .render
                    .len()
                    .saturating_sub(colstart)
                    .min(self.window.width as usize);

                // Nothing to display on this line
                if len == 0 {
                    continue;
                }
                let colend = colstart + len;
                self.stdout
                    .queue(cursor::MoveTo(0, y))?
                    .queue(style::Print(
                        self.editrows[filerow].render[colstart..colend].to_string(),
                    ))?;
            }
        }
        Ok(())
    }

    pub fn draw_status(&mut self) -> crossterm::Result<()> {
        let width = self.window.width as usize;

        let mut status_left = if let Some(filename) = &self.file {
            format!("{} [{} lines]", filename, self.editrows.len())
        } else {
            "[No Name]".to_string()
        };
        status_left.truncate(width);

        let msg_right = format!("{}/{}", self.cursor.y + 1, self.editrows.len());

        let mut status_right = String::new();
        if status_left.len() < self.window.width as usize - msg_right.len() {
            let mut len = status_left.len();
            while len < width {
                if width - len == msg_right.len() {
                    status_right.push_str(&msg_right);
                    break;
                } else {
                    status_right.push(' ');
                    len += 1;
                }
            }
        }
        let status_msg = format!("{}{}", status_left, status_right);

        let color_status = style::Colors::new(style::Color::Black, style::Color::White);
        self.stdout
            .queue(cursor::MoveTo(0, self.window.height))?
            .queue(style::SetColors(color_status))?
            .queue(style::Print(status_msg))?;

        self.stdout.queue(style::ResetColor)?;

        Ok(())
    }

    pub fn draw_message(&mut self) -> crossterm::Result<()> {
        if self.status_time.elapsed() > Duration::from_secs(5) {
            self.status_msg.clear();
            return Ok(());
        }
        let color_status = style::Colors::new(style::Color::Black, style::Color::White);
        let status_help: String = self
            .status_msg
            .chars()
            .take(self.window.width as usize)
            .collect();
        self.stdout
            .queue(style::SetColors(color_status))?
            .queue(cursor::MoveTo(0, self.window.height + 1))?
            .queue(style::Print(status_help))?;
        self.stdout.queue(style::ResetColor)?;
        Ok(())
    }

    pub fn _set_status(&mut self, message: &str) {
        self.status_time = time::Instant::now();
        self.status_msg = message.to_string();
    }

    pub fn show_welcome(&mut self, row: u16) -> crossterm::Result<()> {
        let mut welcome = format!("Kilo-rs version {VERSION}");
        welcome.truncate(self.window.width as usize);
        if welcome.len() < self.window.width as usize {
            let left = ((self.window.width as usize - welcome.len()) / 2) as u16;
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(style::Print("~".to_string()))?
                .queue(cursor::MoveTo(left, row))?
                .queue(style::Print(welcome))?;
        } else {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(style::Print(welcome))?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> crossterm::Result<()> {
        self.stdout.flush()
    }

    pub fn position(&self) -> Position {
        self.cursor.clone()
    }

    pub fn _read_pos() -> crossterm::Result<Position> {
        let (x, y) = crossterm::cursor::position()?;
        Ok(Position::new(x, y))
    }

    /*
     * pos.y does not refer to the position of the cursor on the screen.
     * It refers to the position of the cursor within the text file. To
     * position the cursor on the screen, subtract rowoff from the value
     * of pos.cy.
     */
    pub fn move_to(&mut self, pos: Position) -> crossterm::Result<()> {
        self.stdout.queue(cursor::MoveTo(
            pos.rx - self.coloff as u16,
            pos.y - self.rowoff as u16,
        ))?;
        Ok(())
    }

    pub fn move_cursor(&mut self, key: CursorKey) {
        match key {
            CursorKey::Left => {
                if self.cursor.x != 0 {
                    self.cursor.x -= 1;
                } else if self.cursor.y > 0 {
                    // Goto the end of last line if cursor isn't already at the top
                    self.cursor.y -= 1;
                    self.cursor.x = self.editrows[self.cursor.y as usize].chars.len() as u16;
                }
            }
            CursorKey::Right => {
                /* Find editrow index based on if data is available at the row.
                 * Check if data is available at the editrow
                 */
                #[allow(clippy::comparison_chain)]
                if (self.cursor.y as usize) < self.editrows.len() {
                    let idx = self.cursor.y as usize;
                    // limit scrollng to the right
                    if (self.cursor.x as usize) < self.editrows[idx].chars.len() {
                        self.cursor.x += 1;
                    } else if (self.cursor.x as usize) == self.editrows[idx].chars.len() {
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                }
            }
            CursorKey::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            CursorKey::Down => {
                // allow the cursor to advance past the bottom of the screen, but
                // not past the bottom of the file.
                if (self.cursor.y as usize) < self.editrows.len() {
                    self.cursor.y += 1
                }
            }
            CursorKey::PageUp | CursorKey::PageDown => {
                let direction = if key == CursorKey::PageUp {
                    self.cursor.y = self.rowoff as u16;
                    CursorKey::Up
                } else {
                    let screenrows = self.window.height as usize;
                    self.cursor.y = (self.rowoff + screenrows - 1).min(self.editrows.len()) as u16;
                    CursorKey::Down
                };
                let times = self.window.height as usize;
                for _ in 0..times {
                    self.move_cursor(direction);
                }
            }
            CursorKey::Home => {
                self.cursor.x = 0;
            }
            CursorKey::End => {
                let cy = self.cursor.y as usize;
                if cy < self.editrows.len() {
                    self.cursor.x = self.editrows[cy].chars.len() as u16;
                }
            }
            CursorKey::Delete | CursorKey::Backspace | CursorKey::Enter | CursorKey::Tab => {}
        }
        // Find the number of characters on the editrow
        let rowlen = if self.cursor.y as usize >= self.editrows.len() {
            0
        } else {
            self.editrows[self.cursor.y as usize].chars.len() as u16
        };
        self.cursor.x = self.cursor.x.min(rowlen);
    }

    /*
     * Check if the cursor has moved outside of the visible window, if so,
     * adjust E.rowoff so that the cursor is just inside the visible window.
     * Call this function right before the screen is refreshed.
     */
    pub fn scroll(&mut self) {
        self.cursor.rx = if (self.cursor.y as usize) < self.editrows.len() {
            self.editrows[self.cursor.y as usize].cx_to_rx(self.cursor.x)
        } else {
            0
        };

        // Check if cursor is above the visible window
        if (self.cursor.y as usize) < self.rowoff {
            self.rowoff = self.cursor.y as usize;
        }

        /* Check if cursor is past the bottom of the visible window. 'rowoff' refers
         * to the what is at the 'top' of the screen. And 'window.height' needs to be
         * used to figure out the bottom of the screen.
         */
        if (self.cursor.y as usize) >= self.rowoff + (self.window.height as usize) {
            self.rowoff = self.cursor.y as usize - self.window.height as usize + 1;
        }

        if (self.cursor.rx as usize) < self.coloff {
            self.coloff = self.cursor.rx as usize;
        }
        if (self.cursor.rx as usize) >= (self.coloff + self.window.width as usize) {
            self.coloff = self.cursor.rx as usize - self.window.width as usize
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if (self.cursor.y as usize) == self.editrows.len() {
            self.editrows.push(EditRow::new(String::new()));
        }
        let cy = self.cursor.y as usize;
        let cx = self.cursor.x as usize;
        self.editrows[cy].insert_char(cx, ch);
        self.cursor.x += 1;
    }

    pub fn release(&mut self) -> crossterm::Result<()> {
        let _ = self.clear();
        terminal::disable_raw_mode()
    }
}
