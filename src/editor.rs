use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use std::fmt::Display;

use crate::input::*;
use crate::screen::*;

pub struct Editor {
    input: Input,
    screen: Screen,
}

impl Editor {
    pub fn new() -> crossterm::Result<Self> {
        Ok(Self {
            input: Input::new(),
            screen: Screen::new()?,
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
            let result = self.input.read();
            match result {
                Ok(event) => match event {
                    EventType::Key(key) => {
                        // println!("{:?}\r", key);
                        if key.code == KeyCode::Char('q')
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break;
                        }
                    }
                },
                Err(e) => {
                    self.die("Failed to read event", e);
                }
            }
        }
        self.screen.release()
    }
}
