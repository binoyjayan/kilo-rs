use crossterm::cursor;
use crossterm::style::Print;
use crossterm::terminal;
use crossterm::QueueableCommand;
use std::io;
use std::io::Write;

use crate::cursor::*;
use crate::events::*;
use crate::window::*;

pub struct Screen {
    stdout: io::Stdout,
    window: Window,
    cursor: Position,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Screen {
    pub fn new() -> crossterm::Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            stdout: io::stdout(),
            window: Window::new(columns, rows),
            cursor: Position::new(1, 0),
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

    pub fn draw_rows(&mut self) -> crossterm::Result<()> {
        for y in 0..self.window.height {
            if y == self.window.height / 3 {
                self.show_welcome(y)?;
            } else {
                self.stdout
                    .queue(cursor::MoveTo(0, y))?
                    .queue(Print("~".to_string()))?;
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

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.clear()?;
        self.draw_rows()?;
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

    pub fn move_to(&mut self, pos: Position) -> crossterm::Result<()> {
        self.stdout.queue(cursor::MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    pub fn move_cursor(&mut self, key: EditorKey) {
        match key {
            EditorKey::Left => {
                if self.cursor.x != 1 {
                    self.cursor.x -= 1;
                }
            }
            EditorKey::Right => {
                if self.cursor.x < self.window.width - 1 {
                    self.cursor.x += 1;
                }
            }
            EditorKey::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            }
            EditorKey::Down => {
                if self.cursor.y < self.window.height {
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
            EditorKey::Home | EditorKey::End => {
                let direction = if key == EditorKey::Home {
                    EditorKey::Left
                } else {
                    EditorKey::Right
                };
                for _ in 0..self.window.width {
                    self.move_cursor(direction);
                }
            }
            EditorKey::Delete | EditorKey::Backspace => {}
        }
    }

    pub fn release(&mut self) -> crossterm::Result<()> {
        let _ = self.clear();
        terminal::disable_raw_mode()
    }
}
