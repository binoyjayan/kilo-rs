use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;

use crate::events::*;

pub struct Input;

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn key_event(&self, key: KeyEvent) -> crossterm::Result<KiloEvent> {
        match key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Ok(KiloEvent::Editor(EditorEvent::Quit)),
            KeyEvent { code, .. } => match code {
                KeyCode::Up => Ok(KiloEvent::Cursor(EditorKey::Up)),
                KeyCode::Left => Ok(KiloEvent::Cursor(EditorKey::Left)),
                KeyCode::Down => Ok(KiloEvent::Cursor(EditorKey::Down)),
                KeyCode::Right => Ok(KiloEvent::Cursor(EditorKey::Right)),
                KeyCode::PageUp => Ok(KiloEvent::Cursor(EditorKey::PageUp)),
                KeyCode::PageDown => Ok(KiloEvent::Cursor(EditorKey::PageDown)),
                KeyCode::Home => Ok(KiloEvent::Cursor(EditorKey::Home)),
                KeyCode::End => Ok(KiloEvent::Cursor(EditorKey::End)),
                KeyCode::Delete => Ok(KiloEvent::Cursor(EditorKey::Delete)),
                KeyCode::Backspace => Ok(KiloEvent::Cursor(EditorKey::Backspace)),
                _ => Ok(KiloEvent::Key(key)),
            },
        }
    }

    pub fn read(&self) -> crossterm::Result<KiloEvent> {
        loop {
            match event::read() {
                Ok(event) => match event {
                    Event::Key(key) => return self.key_event(key),
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
