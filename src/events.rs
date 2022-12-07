use crossterm::event::KeyEvent;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EditorKey {
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Delete,
    Backspace,
}

#[derive(Copy, Clone)]
pub enum EditorEvent {
    Quit,
}

pub enum KiloEvent {
    Key(KeyEvent),
    Editor(EditorEvent),
    Cursor(EditorKey),
}
