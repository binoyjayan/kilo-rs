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

    // Check if ch is a valid file name character
    pub fn is_valid_file_char(ch: char) -> bool {
        (' '..='~').contains(&ch)
    }

    // Decode key, return None if it can be ignored
    pub fn key_event(&self, key: KeyEvent) -> Option<EditorEvent> {
        match key {
            // Control keys
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => match ch {
                'q' => Some(EditorEvent::Control(ControlEvent::Quit)),
                'h' => Some(EditorEvent::Cursor(CursorKey::Backspace)),
                's' => Some(EditorEvent::Control(ControlEvent::Save)),
                'f' => Some(EditorEvent::Control(ControlEvent::Find)),
                'l' => Some(EditorEvent::Control(ControlEvent::Line)),
                _ => None,
            },
            // Cursor and character keys
            KeyEvent {
                code,
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            } => match code {
                KeyCode::Up => Some(EditorEvent::Cursor(CursorKey::Up)),
                KeyCode::Left => Some(EditorEvent::Cursor(CursorKey::Left)),
                KeyCode::Down => Some(EditorEvent::Cursor(CursorKey::Down)),
                KeyCode::Right => Some(EditorEvent::Cursor(CursorKey::Right)),
                KeyCode::PageUp => Some(EditorEvent::Cursor(CursorKey::PageUp)),
                KeyCode::PageDown => Some(EditorEvent::Cursor(CursorKey::PageDown)),
                KeyCode::Home => Some(EditorEvent::Cursor(CursorKey::Home)),
                KeyCode::End => Some(EditorEvent::Cursor(CursorKey::End)),
                KeyCode::Delete => Some(EditorEvent::Cursor(CursorKey::Delete)),
                KeyCode::Backspace => Some(EditorEvent::Cursor(CursorKey::Backspace)),
                KeyCode::Enter => Some(EditorEvent::Cursor(CursorKey::Enter)),
                KeyCode::Tab => Some(EditorEvent::Key('\t')),
                KeyCode::Char(ch) => Some(EditorEvent::Key(ch)),
                KeyCode::Esc => Some(EditorEvent::Control(ControlEvent::Escape)),
                _ => None,
            },
            KeyEvent { .. } => None,
        }
    }

    pub fn read(&self) -> crossterm::Result<EditorEvent> {
        loop {
            match event::read() {
                Ok(event) => match event {
                    Event::Key(key) => {
                        if let Some(key) = self.key_event(key) {
                            return Ok(key);
                        }
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
