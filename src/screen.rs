use crossterm::cursor;
use crossterm::style::Print;
use crossterm::terminal;
use crossterm::QueueableCommand;
use std::io;
use std::io::Write;

use crate::cursor::*;
use crate::data::*;
use crate::events::*;
use crate::window::*;

pub struct Screen {
    stdout: io::Stdout,
    window: Window,
    cursor: Position,
    editrow: EditRow,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Screen {
    pub fn new(lines: &[String]) -> crossterm::Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            stdout: io::stdout(),
            window: Window::new(columns, rows),
            cursor: Position::new(0, 0),
            editrow: EditRow::new(Vec::from(lines)),
        })
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
            let filerow = y as usize + self.editrow.rowoff;
            if filerow >= self.editrow.chars.len() {
                if self.editrow.chars.is_empty() && y == self.window.height / 3 {
                    self.show_welcome(y)?;
                } else {
                    self.stdout
                        .queue(cursor::MoveTo(0, y))?
                        .queue(Print("~".to_string()))?;
                }
            } else {
                let colstart = self.editrow.coloff;
                // Handling horizontal scrolling
                let len = self.editrow.chars[filerow]
                    .len()
                    .saturating_sub(colstart)
                    .min(self.window.width as usize);

                // Nothing to display on this line
                if len == 0 {
                    continue;
                }
                let colend = colstart + len;
                self.stdout.queue(cursor::MoveTo(0, y))?.queue(Print(
                    self.editrow.render[filerow][colstart..colend].to_string(),
                ))?;
            }
        }
        Ok(())
    }

    pub fn show_welcome(&mut self, row: u16) -> crossterm::Result<()> {
        let mut welcome = format!("Kilo-rs version {VERSION}");
        welcome.truncate(self.window.width as usize);
        if welcome.len() < self.window.width as usize {
            let left = ((self.window.width as usize - welcome.len()) / 2) as u16;
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print("~".to_string()))?
                .queue(cursor::MoveTo(left, row))?
                .queue(Print(welcome))?;
        } else {
            self.stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(Print(welcome))?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> crossterm::Result<()> {
        self.stdout.flush()
    }

    pub fn position(&self) -> Position {
        self.cursor.clone()
    }

    // pub fn read_pos() -> crossterm::Result<Position> {
    //     let (x, y) = crossterm::cursor::position()?;
    //     Ok(Position::new(x, y))
    // }

    /*
     * pos.y does not refer to the position of the cursor on the screen.
     * It refers to the position of the cursor within the text file. To
     * position the cursor on the screen, subtract rowoff from the value
     * of pos.cy.
     */
    pub fn move_to(&mut self, pos: Position) -> crossterm::Result<()> {
        self.stdout.queue(cursor::MoveTo(
            pos.x - self.editrow.coloff as u16,
            pos.y - self.editrow.rowoff as u16,
        ))?;
        Ok(())
    }

    pub fn move_cursor(&mut self, key: EditorKey) {
        match key {
            EditorKey::Left => {
                if self.cursor.x != 0 {
                    self.cursor.x -= 1;
                } else if self.cursor.y > 0 {
                    // Goto the end of last line if cursor isn't already at the top
                    self.cursor.y -= 1;
                    self.cursor.x = self.editrow.chars[self.cursor.y as usize].len() as u16;
                }
            }
            EditorKey::Right => {
                /* Find editrow index based on if data is available at the row.
                 * Check if data is available at the editrow
                 */
                if (self.cursor.y as usize) < self.editrow.chars.len() {
                    let idx = self.cursor.y as usize;
                    // limit scrollng to the right
                    if (self.cursor.x as usize) < self.editrow.chars[idx].len() {
                        self.cursor.x += 1;
                    } else if (self.cursor.x as usize) == self.editrow.chars[idx].len() {
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                }
            }
            EditorKey::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            EditorKey::Down => {
                // allow the cursor to advance past the bottom of the screen, but
                // not past the bottom of the file.
                if (self.cursor.y as usize) < self.editrow.chars.len() {
                    self.cursor.y += 1
                }
            }
            EditorKey::PageUp | EditorKey::PageDown => {
                let direction = if key == EditorKey::PageUp {
                    EditorKey::Up
                } else {
                    EditorKey::Down
                };
                for _ in 0..self.window.height {
                    self.move_cursor(direction);
                }
            }
            EditorKey::Home => {
                self.cursor.x = 0;
            }
            EditorKey::End => {
                let idx = self.cursor.y as usize;
                self.cursor.x = if (self.cursor.x as usize) < self.editrow.chars[idx].len() {
                    self.editrow.chars[idx].len() as u16
                } else {
                    0
                };
            }
            EditorKey::Delete | EditorKey::Backspace => {}
        }
        // Find the number of characters on the editrow
        let rowlen = if self.cursor.y as usize >= self.editrow.chars.len() {
            0
        } else {
            self.editrow.chars[self.cursor.y as usize].len() as u16
        };
        self.cursor.x = self.cursor.x.min(rowlen);
    }

    /*
     * Check if the cursor has moved outside of the visible window, if so,
     * adjust E.rowoff so that the cursor is just inside the visible window.
     * Call this function right before the screen is refreshed.
     */
    pub fn scroll(&mut self) {
        // Check if cursor is above the visible window
        if (self.cursor.y as usize) < self.editrow.rowoff {
            self.editrow.rowoff = self.cursor.y as usize;
        }

        /* Check if cursor is past the bottom of the visible window. 'rowoff' refers
         * to the what is at the 'top' of the screen. And 'window.height' needs to be
         * used to figure out the bottom of the screen.
         */
        if (self.cursor.y as usize) >= self.editrow.rowoff + (self.window.height as usize) {
            self.editrow.rowoff = self.cursor.y as usize - self.window.height as usize + 1;
        }

        if (self.cursor.x as usize) < self.editrow.coloff {
            self.editrow.coloff = self.cursor.x as usize;
        }
        if (self.cursor.x as usize) >= (self.editrow.coloff + self.window.width as usize) {
            self.editrow.coloff = self.cursor.x as usize - self.window.width as usize
        }
    }

    pub fn release(&mut self) -> crossterm::Result<()> {
        let _ = self.clear();
        terminal::disable_raw_mode()
    }
}
