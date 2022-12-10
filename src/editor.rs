use std::fmt::Display;
use std::fs;
use std::path::Path;

use crate::events::*;
use crate::input::*;
use crate::screen::*;

pub struct Editor {
    input: Input,
    screen: Screen,
}

impl Editor {
    pub fn new() -> crossterm::Result<Self> {
        Self::create(&[])
    }

    pub fn open(file: &Path) -> crossterm::Result<Self> {
        let data = Self::read_file(file);
        let lines: Vec<String> = data.split('\n').map(|s: &str| s.to_string()).collect();
        Self::create(&lines)
    }

    fn read_file(file: &Path) -> String {
        match fs::read_to_string(file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{}: {}", file.to_string_lossy(), e);
                std::process::exit(1);
            }
        }
    }

    pub fn create(lines: &[String]) -> crossterm::Result<Self> {
        Ok(Self {
            input: Input::new(),
            screen: Screen::new(lines)?,
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
                KiloEvent::Key(_key) => {
                    // println!("{:?}\r", key);
                }
                KiloEvent::Cursor(direction) => self.screen.move_cursor(direction),
                KiloEvent::Editor(_) => return Ok(true),
            },
            Err(e) => {
                self.die("Failed to read event", e);
            }
        }
        Ok(false)
    }
}
