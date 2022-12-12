#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorKey {
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
    Enter,
}

#[derive(Debug, Copy, Clone)]
pub enum ControlEvent {
    Quit,
    Save,
    Escape,
}

#[derive(Debug)]
pub enum EditorEvent {
    Key(char),
    Control(ControlEvent),
    Cursor(CursorKey),
}
