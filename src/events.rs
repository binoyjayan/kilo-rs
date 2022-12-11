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
    Tab,
}

#[derive(Debug, Copy, Clone)]
pub enum ControlEvent {
    Quit,
    CtrlH,
    Save,
}

#[derive(Debug)]
pub enum EditorEvent {
    Key(char),
    Control(ControlEvent),
    Cursor(CursorKey),
}
