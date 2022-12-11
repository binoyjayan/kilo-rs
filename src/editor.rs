use std::fmt::Display;
use std::fs;

use crate::events::*;
use crate::input::*;
use crate::screen::*;

pub struct Editor {
    input: Input,
    screen: Screen,
    file: Option<String>,
}

impl Editor {
    pub fn new() -> crossterm::Result<Self> {
        Self::create(&[], None)
    }

    pub fn open(file: &str) -> crossterm::Result<Self> {
        let data = Self::read_file(file);
        let lines: Vec<String> = data.split('\n').map(|s: &str| s.to_string()).collect();
        Self::create(&lines, Some(file.to_string()))
    }

    fn read_file(file: &str) -> String {
        match fs::read_to_string(file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{}: {}", file, e);
                std::process::exit(1);
            }
        }
    }

    pub fn create(lines: &[String], file: Option<String>) -> crossterm::Result<Self> {
        Ok(Self {
            input: Input::new(),
            screen: Screen::new(lines, file.clone())?,
            file,
        })
    }

    pub fn die<T: Display>(&mut self, message: &str, err: T) {
        let _ = self.screen.release();
        eprintln!("{}: {}", message, err);
        std::process::exit(1);
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        self.screen.open()?;

        loop {
            self.screen.refresh()?;
            let pos = self.screen.position();
            self.screen.move_to(pos)?;
            self.screen.flush()?;

            if self.event()? {
                break;
            }
        }
        self.screen.release()
    }

    pub fn event(&mut self) -> crossterm::Result<bool> {
        let result = self.input.read();

        match result {
            Ok(event) => match event {
                EditorEvent::Key(ch) => {
                    self.screen.insert_char(ch);
                }
                EditorEvent::Cursor(direction) => self.screen.move_cursor(direction),
                EditorEvent::Control(ctrl) => match ctrl {
                    ControlEvent::Quit => {
                        let quit_times = self.screen.dec_quit_times();
                        if self.screen.is_dirty() && quit_times > 0 {
                            let msg = format!("WARNING: File has unsaved changes. Press Ctrl-Q {} more time(s) to quit", quit_times);
                            self.screen.set_status(&msg);
                            return Ok(false);
                        } else {
                            return Ok(true);
                        }
                    }
                    ControlEvent::Save => self.save(),
                    ControlEvent::CtrlH => self.screen.del_char(),
                },
            },
            Err(e) => {
                self.die("Failed to read event", e);
            }
        }
        self.screen.reset_quit_times();
        Ok(false)
    }

    pub fn save(&mut self) {
        if let Some(filename) = &self.file {
            let buf = self.screen.rows_to_string();
            let msg = match fs::write(filename, &buf) {
                Ok(_) => {
                    let file_len = buf.as_bytes().len();
                    self.screen.set_dirty(false);
                    format!("{} bytes written to {}", file_len, filename)
                }
                Err(e) => format!("Failed to write to {} - {}", filename, e),
            };
            self.screen.set_status(&msg);
        }
    }
}
