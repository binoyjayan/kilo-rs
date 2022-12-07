use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEvent;

pub struct Input;

pub enum EventType {
    Key(KeyEvent),
}

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read(&self) -> crossterm::Result<EventType> {
        loop {
            match event::read() {
                Ok(event) => match event {
                    Event::Key(key) => {
                        return Ok(EventType::Key(key));
                    }
                    _ => {
                        println!("other event\r");
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
